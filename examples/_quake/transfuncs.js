function from_todo_blog_to_quake_calendar(todos, blogs) {
  let results = [];
  for (let todo of todos) {
    results.push({
      type: "todo",
      id: todo.id,
      title: todo.title,
      content: todo.content,
      start: todo.created_date,
      end: todo.updated_date
    })
  }

  for (let blog of blogs) {
    results.push({
      type: "blog",
      id: blog.id,
      title: blog.title,
      content: blog.description,
      start: blog.created_date,
      end: blog.updated_date
    })
  }

  return results;
}
