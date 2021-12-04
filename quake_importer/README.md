# Quake Importer

- [x] Django / Mezzanine Database
- [ ] OneNote
- [x] Microsoft Todo
- [ ] Apple Calendar
- [x] Apple Notes

## Custom DSL for merge field

## Export

### Microsoft Todo Export

use `https://github.com/quakeworks/microsoft-todo-export` for Microsoft Todo

### Apple Notes

use `https://github.com/quakeworks/readnotes` for build `mac_apt.db`

```
git clone https://github.com/quakeworks/readnotes
pip install -r requirements.txt

python3 -B readnotes.py  --user phodal --input "NoteStore.sqlite" --output .
```

