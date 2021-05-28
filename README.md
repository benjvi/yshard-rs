<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**

- [yshard](#yshard)
- [Getting Started](#getting-started)
- [Implementation](#implementation)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->


# yshard

*Huge yaml files are impossible to understand, so let's split them up!*

yshard is a CLI that takes a single YAML file as input and splits it into separate files, doing a GROUP BY on the user-provided JSON path. It is particularly useful in cases where large complex packages are distributed as a single YAML file, containing multiple YAML documents - as is often the case for Kubernetes manifests. yshard is also useful when you render your own templates into a single output file. 


# Getting Started

Grab the binary for your architecture from [the releases page](https://github.com/benjvi/yshard/releases) (note - a build for Windows is not currently available). 

Get a YAML file you want to split up, for example:

`wget https://raw.githubusercontent.com/argoproj/argo-workflows/stable/manifests/install.yaml`

Now use yshard to split the YAML, in this case we split by the `kind` field of each document, putting documents with the same `kind` in the same file in `output-directory`:

`cat install | yshard -g ".kind" -o output-directory`

Then in the output directory you see: 

```$ ls output-directory
ClusterRole.yml              ConfigMap.yml                Deployment.yml               RoleBinding.yml              ServiceAccount.yml
ClusterRoleBinding.yml       CustomResourceDefinition.yml Role.yml                     Service.yml                  __ungrouped__.yml
```

# Implementation

This CLI is built in Rust, based on [rust-starter](https://github.com/rust-starter/rust-starter). It largely relies on jq-rs for the json manipulation.
