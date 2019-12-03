## CSV TYPES

I have started csv types because I had to deal with third party csv files > 100,000 lines.
The problem has been to determin which types can be found in each column. For example I had a file which contained weather data but in one row wind direction and wind speed have been swaped. My goal with csv types was to write a tool which can find possible types for each column.

## Usage

### Parameter
|short|long|example|description|
|:-:|:-:|:-:|:-:|
|-h|--help||Show this help message and exit|
||--header||File has header|
|-c|--config-file|config.cfg|Add custom types from file|
|-C|--config-file-replace-default|config.cfg|Same as --config-file but replaces default config|
||--max-threads|4|Maximal thread count|
||--assert|"String,int,float"|Returns not matching rows and columns in pattern [row]:[column]:[column]...|

### File with out headers
csv_types < \[csv.file]

### Config File
Config files can contain new definitions for types
[type name] [pattern]

#### Example
```
float 
```


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
