# RBAC

Learning how Kubernetes and Rancher RBAC work. A mix of notes, specific examples and commands to work with.

## Kubernetes RBAC

Source: https://kubernetes.io/docs/reference/access-authn-authz/rbac/

### Setup

The easiest way is through `kind` ([source](https://kind.sigs.k8s.io/docs/user/quick-start/)) since the cluster has RBAC enabled by default

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

Apply any of the following with `kubectl apply -f <yaml>`

#### Roles

This role allows a user to access pods in the `default` namespace

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
