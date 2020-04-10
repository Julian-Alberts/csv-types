## CSV TYPES

I have started csv types because I had to deal with third party csv files > 100,000 lines.
The problem has been to determin which types can be found in each column. For example I had a file which contained weather data but in one row wind direction and wind speed have been swaped. My goal with csv types was to write a tool which can find possible types for each column.

## Usage

### Sub commands

#### match
Return all matching types 

|short|long|example|description|
|:-:|:-:|:-:|:-:|
|-h|--help||Show this help message and exit|
||--header||File has header|
|-c|--config-file|config.cfg|Add custom types from file|
|-C|--config-file-replace-default|config.cfg|Same as --config-file but replaces default config|
||--max-threads|4|Maximal thread count|
|-m|||Machine readable format|

**Example:**
`csv_types match --header --max-threads 2 --config-file ./types.conf`

**Example output:** 
<pre>
| string | string | string | string |
|  float |        |  float |        |
|        |        |    int |        |
</pre>

**Example output with header:**
<pre>
|  col 1 |  col 2 |  col 3 |  col 4 |
=====================================
| string | string | string | string |
|  float |        |  float |        |
|        |        |    int |        |
</pre>

#### assert
Check if types match columns

|short|long|example|description|
|:-:|:-:|:-:|:-:|
|-h|--help||Show this help message and exit|
||--header||File has header|
|-c|--config-file|config.cfg|Add custom types from file|
|-C|--config-file-replace-default|config.cfg|Same as --config-file but replaces default config|
||--max-threads|4|Maximal thread count|
|-m|||Machine readable format|

**Example**
`csv_types assert --header --max-threads 2 --config-file ./types.conf string,float,int`


### Config File
Config files can contain new definitions for types
[type name] [pattern]

#### Example
```
float [-+]?(?:(?:\d+(?:\.\d*)?)|\.\d+)
```
