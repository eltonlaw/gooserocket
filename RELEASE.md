# Release Log

## 0.1.1

- EC2 ImageBuilder pipeline not that useful for one off dev instance images, switched it back to more ad-hoc with an install script instead.
- System and python libs encoded in `*.nix` files, just need to set up `nix`.

## 0.1.0

- Deploy EC2 instance in cloudformation stack. Using a `assets/jupyter_notebook.yaml` file. Need to update this to generate the yaml automatically so we can template out certain bits and share values as more stacks are built.
