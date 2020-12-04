# TODO

### High-Priority Tasks

- [ ] audit database indexes for all models
    - [ ] add indexes for notes and media
    - [ ] add indexes for links
- [ ] basic keyword search
- [ ] add model to track links
    - [ ] allow linking from markdown content
    - [ ] track links to internal resources (notes / media)
    - [ ] track links to external resources

#### Pre-v0.2

- [ ] add search bar to dashboard
- [ ] add modal component
    - [ ] keyboard shortcut info modal (trigger w/ cmd-?)
- [ ] section headers
    - [ ] breadcrumbs
    - [ ] action button bars
- [ ] improve list and list item components
- [ ] pagination for note + media listings
- [ ] task list item dragging / reordering & dependencies
- [ ] manage code/preview splits (show full page, adjust split sizes)
- [ ] improve settings page (i.e. site list + page list)
- [ ] [infra] logging + metrics (set up prometheus + fluentd agents)
- [ ] [infra] save access logs for NLB to S3 (and process w/ fluentd?)
- [ ] [infra] use Fargate & Tailscale / Argo to allow SSH access from outside AWS
    - [ ] limit SSH access for app instances to VPN subnet only
- [ ] [infra] set `skip_final_snapshot=false` for RDS
    
#### Pre-v1.0

- [ ] refactor upload flow
- [ ] improve test coverage (FE & BE)
    - [ ] [ci-tooling] code coverage reports
- [ ] job system improvements
    - [ ] dependent tasks (use pg trigger to unblock dependents)
    - [ ] scheduled tasks
    - [ ] one-shot tasks (scripts & migrations)
    - [ ] refactor to share structs between app and worker
    - [ ] write tests for job subscription + retries (make sure jobs retry on failure!)
- [ ] schedule lambda to rotate KMS keys for RDS
- [ ] schedule lambda to rotate RDS root password
- [ ] use IAM authentication to access RDS from app instances
- [ ] disable public IPs for app instances (use egress-only internet gateway)
- [ ] make sure EBS volumes are encrypted (or remove them all)
- [ ] make sure all S3 buckets use encryption w/ custom KMS key
- [ ] add S3 lifecycle rules where appropriate
- [ ] audit IAM roles and lock them down where appropriate
- [ ] security group: remove outbound `:80` & `:443` from app instances
- [ ] clean up remaining infra TODOs

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
- audit infra security
    - [x] schedule lambda to refresh TLS cert every day
- content export to HTML
    - [x] render pages
    - [x] render list page
    - [x] save exported site to S3


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
