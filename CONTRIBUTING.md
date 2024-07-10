# Contributing

## Releasing a New Version to PyPI

```bash
rye version -b minor
git commit -am "Release version x.y.z"
rye build
```

Make sure the built package works:

```bash
python -m venv .venvtmp
source .venvtmp/bin/activate
pip install dist/your-package-0.3.0-py3-none-any.whl
(test your package here)
deactivate
rm -rf .venvtmp
```

Then publish the package to PyPI:

```bash
rye publish
```

Then publish the tag to GitHub:

```bash
git tag -a x.y.z -m "Version x.y.z"
git push --tags
```

Finally, create a new release on GitHub:

* Create a new release on GitHub by clicking "Create a new release"
* From the tag dropdown, choose the tag you just created
* Click "Generate release notes" to auto-populate the release notes
* Copy in whatever notes you have from the `CHANGELOG.md` file
* Revise the notes as needed
* Attach the distribution package: drag and drop the `.tar.gz` and `.whl` files from the `dist/` directory
* Click "Publish release"
