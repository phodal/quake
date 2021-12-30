Patterns

- Internal DSL
- External DSL
- Declarative Programming
- Annotation

## Papers

###

> WorkflowDSL: Scalable Workflow Execution with Provenance

Link: [https://www.diva-portal.org/smash/get/diva2:1149093/FULLTEXT01.pdf](https://www.diva-portal.org/smash/get/diva2:1149093/FULLTEXT01.pdf)

```
input_tasks = %gremlin g.V().workflow('brainAtlas').out('run')
  .trial().limit(1).out('next').values('name')

output_tasks = %gremlin g.V().workflow('brainAtlas').out('run')
.trial().limit(1).repeat(out('next')).until(outE('next').count() .is(0)).dedup().values('name')
```