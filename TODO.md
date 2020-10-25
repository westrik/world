# TODO

### High-Priority Tasks

- [ ] audit database indexes for all models
- [ ] content export to HTML
    - [ ] create site with HTML exports
    - [ ] save exported site to S3
- [ ] rename `library_item` to `media_item`
    - [ ] refactor upload flow?

#### Backlog

- [ ] basic search by name
- [ ] add model to track internal links
    - [ ] allow linking from markdown content to media
    - [ ] fix indexes for notes and media
- [ ] add model to track links to external resources
- [ ] add model to track links to external resources
- [ ] write tests for job subscription + retries (make sure jobs retry on failure!)
- [ ] UI improvements
    - [ ] dashboard (at least: add search bar)
    - [ ] section headers w/ action button bars
    - [ ] modal component
        - [ ] keyboard shortcut info modal (trigger w/ cmd-?)
    - [ ] improve list and list item components
    - [ ] pagination for note / task / media listings
    - [ ] task list item dragging / reordering & dependencies
    - [ ] manage code/preview splits (show full page, adjust split sizes)
    - [ ] improve keyboard shortcuts for general navigation
    - [ ] improve keyboard shortcuts for task manipulation
    - [ ] improve keyboard shortcuts for note viewing & editing
- [ ] logging + metrics (set up prometheus + fluentd agents)
    - [ ] S3 & CloudWatch for logs
    - [ ] CloudWatch for metrics?
- [ ] audit infra security before v0.2 release
    - [ ] disable public IPs for app instances (use egress-only internet gateway)
    - [ ] security group: remove outbound `:80` & `:443` from app instances
    - [ ] use Fargate & Tailscale / Argo to allow SSH access from outside AWS
    - [ ] security group: limit SSH access for app instances to VPN subnet only
    - [ ] make sure EBS volumes are encrypted (or remove them all)
    - [ ] make sure all S3 buckets use encryption w/ custom KMS key
    - [ ] add S3 lifecycle rules where appropriate
    - [ ] save access logs for NLB to S3
    - [ ] schedule lambda to rotate KMS keys for RDS
    - [ ] schedule lambda to rotate RDS root password
    - [ ] create systemd task to periodically refresh secrets on app instances
    - [ ] schedule lambda to refresh TLS cert every 2 weeks (?)
    - [ ] use IAM authentication to access RDS from app instances
    - [ ] audit IAM roles and lock them down where appropriate
    - [ ] set `skip_final_snapshot=false` for RDS
- [ ] job system improvements
    - [ ] dependent tasks (use pg trigger to unblock dependents)
    - [ ] scheduled tasks
    - [ ] one-shot tasks (scripts & migrations)
- [ ] export content to markdown (on backend)
- [ ] export content to PDF
    - [ ] figure out how to make tectonic include dependencies at build time (?)
- [ ] clear cloudfront cookies on sign-out?
- [ ] improve test coverage (FE & BE)
- [ ] make lambda to scrape web pages
- [ ] [local dev] make nginx return 504 if backend is missing, then do retries on frontend
- [ ] [ci-tooling] code coverage reports
- [ ] sandboxed auth system for IoT devices (e.g. desk clock)


---------

# Notes


#### `audit database indexes for all models`:

Tables to audit:

- [ ] sessions
- [ ] note_versions
- [ ] notes
- [ ] tasks
- [ ] library_item_versions
- [ ] library_items
- [ ] site_pages
- [ ] jobs

**sessions**:
- Access patterns:
- Indexes to add:

**note_versions**:
- Access patterns:
- Indexes to add:

**notes**:
- Access patterns:
- Indexes to add:

**tasks**:
- Access patterns:
- Indexes to add:

**library_item_versions**:
- Access patterns:
- Indexes to add:

**library_items**:
- Access patterns:
- Indexes to add:

**site_pages**:
- Access patterns:
- Indexes to add:

**jobs**:
- Access patterns:
- Indexes to add:
