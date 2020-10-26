# TODO

### High-Priority Tasks

- [ ] content export to HTML
    - [ ] create site with HTML exports
    - [ ] save exported site to S3
- [ ] refactor upload flow
- [ ] audit database indexes for all models

#### Backlog

- [ ] basic search by name
- [ ] add model to track internal links
    - [ ] allow linking from markdown content to media
    - [ ] fix indexes for notes and media
- [ ] add model to track links to internal resources (notes / media)
- [ ] add model to track links to external resources
- [ ] UI improvements
    - [ ] modal component
        - [ ] keyboard shortcut info modal (trigger w/ cmd-?)
    - [ ] dashboard (at least add search bar)
    - [ ] section headers
        - [ ] breadcrumbs
        - [ ] action button bars
    - [ ] improve list and list item components
    - [ ] pagination for note + media listings
    - [ ] task list item dragging / reordering & dependencies
    - [ ] manage code/preview splits (show full page, adjust split sizes)
    - [ ] improve settings page (i.e. site list + page list)
- [ ] logging + metrics (set up prometheus + fluentd agents)
    - [ ] S3 & CloudWatch for logs
    - [ ] CloudWatch for metrics?
- [ ] job system improvements
    - [ ] dependent tasks (use pg trigger to unblock dependents)
    - [ ] scheduled tasks
    - [ ] one-shot tasks (scripts & migrations)
    - [ ] refactor to share structs between app and worker
    - [ ] write tests for job subscription + retries (make sure jobs retry on failure!)
- [ ] improve test coverage (FE & BE)
    - [ ] [ci-tooling] code coverage reports
- [ ] audit infra security before v0.2 release
    - [ ] disable public IPs for app instances (use egress-only internet gateway)
    - [ ] security group: remove outbound `:80` & `:443` from app instances
    - [ ] make sure EBS volumes are encrypted (or remove them all)
    - [ ] make sure all S3 buckets use encryption w/ custom KMS key
    - [ ] add S3 lifecycle rules where appropriate
    - [ ] save access logs for NLB to S3
    - [ ] create systemd task to periodically refresh secrets on app instances
    - [ ] schedule lambda to refresh TLS cert every 2 weeks (?)
    - [ ] schedule lambda to rotate KMS keys for RDS
    - [ ] schedule lambda to rotate RDS root password
    - [ ] use IAM authentication to access RDS from app instances
    - [ ] use Fargate & Tailscale / Argo to allow SSH access from outside AWS
    - [ ] security group: limit SSH access for app instances to VPN subnet only
    - [ ] audit IAM roles and lock them down where appropriate
    - [ ] clean up remaining infra TODOs
    - [ ] set `skip_final_snapshot=false` for RDS

#### Post-v1.0

- [ ] [local dev] make nginx return 504 if backend is missing, then do retries on frontend
- [ ] export content to markdown (on backend)
- [ ] export content to LaTeX (-> PDF)
    - [ ] figure out how to make tectonic include dependencies at build time (?)
- [ ] clear cloudfront cookies on sign-out?
- [ ] improve keyboard shortcuts for task manipulation
- [ ] improve keyboard shortcuts for note viewing & editing
- [ ] make lambda to scrape web pages
- [ ] sandboxed auth system for IoT devices (e.g. desk clock)

#### Done 🎉

- [x] nav menu improvements
- [x] improve keyboard shortcuts for general navigation
- job system improvements
    - [x] diesel connection pool for worker tasks
- [x] rename `library_item` to `media_item`


---------

# Notes


#### `audit database indexes for all models`:

Tables to audit:

- [ ] sessions
- [ ] note_versions
- [ ] notes
- [ ] tasks
- [ ] media_item_versions
- [ ] media_items
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

**media_item_versions**:
- Access patterns:
- Indexes to add:

**media_items**:
- Access patterns:
- Indexes to add:

**site_pages**:
- Access patterns:
- Indexes to add:

**jobs**:
- Access patterns:
- Indexes to add:
