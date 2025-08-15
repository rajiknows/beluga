# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2025-07-10

### Added

- Initial release of Beluga!
- **Project Initialization**: Create a new static site with `beluga init <site-name>`.
- **Markdown Processing**: Converts Markdown files from the `site/src` directory into HTML.
- **HTML Templating**: Uses corresponding templates from the `site/templates` directory to render the final pages.
- **Live Development Server**: The `beluga watch` command starts a local server at `http://127.0.0.1:42069` and automatically rebuilds the site on file changes.
- **Simplified Content Structure**: Supports a flat file structure for content. All markdown files reside directly in `site/src` and are mapped to templates with the same name in `site/templates`.
- **Basic Page Navigation**: HTML templates include a simple navigation menu to switch between generated pages.
