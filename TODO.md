### Tasks

- [x] write script to set up local development environment
- [x] fix cloudfront cookie authentication
- [x] update LI endpoint to include latest LIV (w/ asset URL)
- [ ] write tests for job subscription + retries
- [ ] dependent jobs (use pg trigger to unblock dependents)
- [ ] call cloudfront auth endpoint on page mount (make sure to clear existing cookies)
- [ ] update front-end to display images + links to non-image files
- [ ] content export to HTML
    - [ ] create site with HTML exports
    - [ ] save exported site to S3
- [ ] allow linking from markdown content to library items

#### backlog

- [ ] content export to PDF
    - [ ] figure out how to make tectonic include dependencies at build time (?)
- [ ] clear cloudfront cookies on sign-out
