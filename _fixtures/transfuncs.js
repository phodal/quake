function from_todo_blog_to_quake_calendar(todos, blogs) {
  let results = [];

  for (let todo of todos) {
    results.push({
      title: todo.title,
      content: todo.content,
      start_time: todo.created_date,
      end_time: todo.updated_date
    })
  }

  for (let blog of blogs) {
    results.push({
      title: blog.title,
      content: blog.content,
      start_time: blog.created_date,
      end_time: blog.updated_date
    })
  }

  return results
}
