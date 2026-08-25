# markdown-diagram
Create diagrams in markdown and render to svg
Must be very simple intuitive syntax to hand code diagrams

| Syntax         | Meaning |
| ---            | --- |
| ` ```diagram ` | Indicates a section that includes a diagram |
| `-> \|->` | Arrow |
| `-v`           | Down arrow |
| `-^`           | Up arrow |
| `[]`           | Box |

## Rules
- Each arrow must start and end in an object

## Todo
[] Models should be able to understand and draw diagrams according to the rules
[] Precommit hook to generate readme with image
[] Readme visualisation with the diagram rendered automatically? Or render on cli?
[] Different styles for drawing ( cartoon, formal ) keep it to a minimum set with styling
[] webservice for drawing and downloading diagrams
