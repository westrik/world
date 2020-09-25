- [x] refactor ASG stuff
- [x] add green ASG
- [ ] set up codedeploy to do blue/green stuff

Turns out that CodeDeploy + blue/green deployments + Terraform is not a nice trio to work with.

CodeDeploy supports blue/green deployments, but it clones the ASG in a way that confuses Terraform.

I think Terraform should manage the LB, but not the ASGs. Then I need some custom system to handle instance and application deploys.
