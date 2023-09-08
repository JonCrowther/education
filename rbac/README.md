# RBAC

Learning how Kubernetes and Rancher RBAC work. A mix of notes, specific examples and commands to work with.

## Kubernetes RBAC

Source: https://kubernetes.io/docs/reference/access-authn-authz/rbac/

### Setup

The easiest way is through `kind` ([source](https://kind.sigs.k8s.io/docs/user/quick-start/)) since the cluster has RBAC enabled by default.

Run `kind create cluster`. Once done, run `kubectl api-resources` and see:

```
NAME                              SHORTNAMES   APIVERSION                             NAMESPACED   KIND
...
clusterrolebindings                            rbac.authorization.k8s.io/v1           false        ClusterRoleBinding
clusterroles                                   rbac.authorization.k8s.io/v1           false        ClusterRole
rolebindings                                   rbac.authorization.k8s.io/v1           true         RoleBinding
roles                                          rbac.authorization.k8s.io/v1           true         Role
...
```

### Summary

K8s RBAC has 4 objects (as shown above):

1. Roles - Grant permissions on a namespaced resource
3. ClusterRoles - Grant permissions on a non-namespaced resource or to a resource across multiple namespaces
4. RoleBindings - Attach a role (or cluster role) to a user in a specific namespace
5. ClusterRoleBindings - Attach a cluster role to a user

### Examples

Apply any of the following with `kubectl apply -f <yaml>`.

#### RoleBindings

##### 1. Create a role

This role allows a user to access pods in the `default` namespace.

<details> 
  <summary>pod_read_role.yaml</summary>

```
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  namespace: default
  name: pod-reader
rules:
- apiGroups: [""] # "" indicates the core API group
  resources: ["pods"]
  verbs: ["get", "watch", "list"]

```

</details>


##### 2. Assign the role to a user

This role binding assigns the user `jono` to the above pod-reader role in the default namespace.

<details>
  <summary>pod_read_role_binding.yaml</summary>

```
apiVersion: rbac.authorization.k8s.io/v1
# This role binding allows "jono" to read pods in the "default" namespace.
# You need to already have a Role named "pod-reader" in that namespace.
kind: RoleBinding
metadata:
  name: read-pods
  namespace: default
subjects:
# You can specify more than one "subject"
- kind: User
  name: jono # "name" is case sensitive
  apiGroup: rbac.authorization.k8s.io
roleRef:
  # "roleRef" specifies the binding to a Role / ClusterRole
  kind: Role #this must be Role or ClusterRole
  name: pod-reader # this must match the name of the Role or ClusterRole you wish to bind to
  apiGroup: rbac.authorization.k8s.io
```

</details>

##### 3. Results

After applying both, check they work by doing:

```
> kubectl get pods --as=jono
No resources found in default namespace.

> kubectl get pods --as=bea
Error from server (Forbidden): pods is forbidden: User "bea" cannot list resource "pods" in API group "" in the namespace "default"
```

As you can see, the user `jono` has the ability to list pods in the default namespace, while `bea` does not.

Note: All roles are permissive, not restrictive.

#### ClusterRoleBindings

##### 1. Create ClusterRole

Create a ClusterRole that allows access to configmaps.

<details>
  <summary>configmap_cluster_role.yaml</summary>

```
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  # "namespace" omitted since ClusterRoles are not namespaced
  name: configmap-reader
rules:
- apiGroups: [""]
  resources: ["configmaps"]
  verbs: ["get", "watch", "list"]
```
</details>

##### 2. Assign the ClusterRole to a user

The ClusterRoleBinding assigns the ClusterRole to the user `jono`.

<details>
  <summary>global_configmap_cluster_role_binding.yaml</summary>

```
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: read-configmaps-global
subjects:
- kind: User
  name: jono # Name is case sensitive
  apiGroup: rbac.authorization.k8s.io
roleRef:
  kind: ClusterRole
  name: configmap-reader
  apiGroup: rbac.authorization.k8s.io
```
</details>

##### 3. Assign the ClusterRole to a user using a RoleBinding

Assigning a ClusterRole to a user via a RoleBinding allows the ClusterRole to be namespace restricted. The below gives the user `bea` the ability to see configmaps only in the `default` namespace.

<details>
  <summary>configmap_role_binding.yaml</summary>

```
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: read-configmaps
  namespace: default
subjects:
- kind: User
  name: bea # Name is case sensitive
  apiGroup: rbac.authorization.k8s.io
roleRef:
  kind: ClusterRole
  name: configmap-reader
  apiGroup: rbac.authorization.k8s.io
```
</details>

##### 4. Results

After applying the 3 files, check they work by doing:

```
> kubectl get cm --as bea
NAME               DATA   AGE
kube-root-ca.crt   1      11m

> kubectl get cm --as bea -A
Error from server (Forbidden): configmaps is forbidden: User "bea" cannot list resource "configmaps" in API group "" at the cluster scope

> kubectl get cm --as jono -A
NAMESPACE            NAME                                                   DATA   AGE
default              kube-root-ca.crt                                       1      11m
...
```

The user `jono` has access to configmaps cluster wide, whereas user `bea` has access to configmaps only in the `default` namespace.
