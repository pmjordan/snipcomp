
text before first snip

```yaml #s1
line 1 in s1
line 2 in s2
```
text between 1 and 2

```yaml #s2
metadata: 
  creation-date: 2024-04-14
  date-updated: 2024-05-01
  status: developmental  
```

Data provided within metadata, wherever it appears, MAY be ignored by
TOSCA Orchestrators and SHOULD NOT affect runtime behavior.
follows:

```yaml #s3
description: <description_string>
```

```yaml #s4
description: This is an example of a single line description (no folding). 
```

The following shows a multiline flow example:

```yaml #s5
description: "A multiline description 
using a quoted string"
```

The YAML *folded* format may also be used for multiline descriptions,
which *folds* line breaks as space characters:

```yaml #s6
description: >
  This is an example of a multi-line description using YAML. It permits for line        
  breaks for easier readability...

  if needed.  However, (multiple) line breaks are folded into a single space   
  character when processed into a single string value.
```

specify the TOSCA version used within the TOSCA file as follows:

```yaml
tosca_definitions_version: <tosca_version> 
```
using the TOSCA Version 2.0 specification:

```yaml #s8
tosca_definitions_version: tosca_2_0
```

