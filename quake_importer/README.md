# Quake Importer

- [ ] Django / Mezzanine Database
- [ ] OneNote
- [ ] Microsoft Todo
- [ ] Apple Calendar
- [ ] Apple

## Custom DSL for merge field

## Export

### Microsoft Todo Export

https://support.microsoft.com/en-us/office/exporting-your-microsoft-to-do-account-d286b243-affb-4db4-addc-162e16588943

### Apple Notes

use `https://github.com/quakeworks/readnotes` for build `mac_apt.db`

```
git clone https://github.com/quakeworks/readnotes
pip install -r requirements.txt

python3 -B readnotes.py  --user phodal --input "NoteStore.sqlite" --output .
```

