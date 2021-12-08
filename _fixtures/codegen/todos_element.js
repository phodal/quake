const tl_show_timeline = async (context, commands) => {
  const el = document.createElement('quake-calendar-timeline');

  let todos = await Quake.query('todo');
  let blogs = await Quake.query('blog');
  let data = from_todo_blog_to_quake_calendar(todos, blogs);

  el.setAttribute('entries', JSON.stringify({
    items: ['blog', 'todo']
  }));
  el.setAttribute('data', JSON.stringify(data));

  return el;
}

