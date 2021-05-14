<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**

- [yshard](#yshard)
- [Getting Started](#getting-started)
- [Implementation](#implementation)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->


# yshard

*Huge yaml files are impossible to understand, so let's split them up!*

yshard is a CLI that takes a single YAML file as input and splits it into separate files, doing a GROUP BY on the user-provided JSON path. It is particularly useful in cases where large complex packages are distributed as a single YAML file, containing multiple YAML documents - as is often the case for Kubernetes manifests. It can also be useful to run yshard when you render a complex set of your own templates into a single output file. 


# Getting Started

# Implementation

This CLI is built in Rust, based on [rust-starter](https://github.com/rust-starter/rust-starter). It largely relies on jq-rs for the json manipulation.
