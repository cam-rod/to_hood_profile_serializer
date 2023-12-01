# Toronto Neighbourhood Profile Serializer

A short script that converts Excel spreadsheets of Toronto neighbourhood profiles (
from [Toronto Open Data](https://open.toronto.ca)) into JSON files. Wrote it to explore the 2021
census data. That also means it doesn't actually implement the proper data exploration API, but
that's not important.

## Instructions

1. Visit the Toronto Open Data page
   for [Neighbourhood Profiles](https://open.toronto.ca/dataset/neighbourhood-profiles/) and
   download the XLSX file you want to parse.
2. Place the file in this folder and rename it `neighbourhood-profiles-<YEAR>.xlsx`.
3. Build this crate, and run for this year:

```bash
to_hood_profile_serializer <YEAR>
```

The output will be placed into `neighbourhood-profiles-<YEAR>.json`. Use `-r/--raw` option if you
don't want pretty-printing.

## License

This code is available under the MIT licence.