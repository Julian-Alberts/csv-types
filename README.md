## CSV TYPES

I have started csv types because I had to deal with third party csv files > 100,000 lines.
The problem has been to determin which types can be found in each column. For example I had a file which contained weather data but in one row wind direction and wind speed have been swaped. My goal with csv types was to write a tool which can find possible types for each column.

## Usage

### File with out headers
csv_types < \[csv.file]


**Example output:** 
<pre>
| string | string | string | string |
|  float |        |  float |        |
|        |        |    int |        |
</pre>
### File with headers
csv_types --header < \[csv.file]

**Example output:**
<pre>
|  col 1 |  col 2 |  col 3 |  col 4 |
=====================================
| string | string | string | string |
|  float |        |  float |        |
|        |        |    int |        |
</pre>

## Planed Features
- machine readable output
- overriding existing type definition
