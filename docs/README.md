# safe-clean Documentation

This directory contains the HTML documentation for safe-clean, which is automatically deployed to GitHub Pages.

## Setup GitHub Pages

To enable GitHub Pages for this repository:

1. Go to repository **Settings** → **Pages**
2. Under "Build and deployment":
   - **Source**: Select "GitHub Actions"
3. The workflow will automatically deploy when changes are pushed to the `main` branch

## Local Testing

To test the documentation locally:

```bash
# Navigate to the docs directory
cd docs

# Start a simple HTTP server
python3 -m http.server 8080

# Open browser to http://localhost:8080
```

## Deployment

The documentation is automatically deployed using GitHub Actions workflow (`.github/workflows/deploy-pages.yml`) when:
- Changes are pushed to the `main` branch
- The workflow is manually triggered from the Actions tab

The deployed site will be available at: `https://npsg02.github.io/safe-clean/`

## Files

- `index.html` - Main documentation page with all features, usage, and examples
