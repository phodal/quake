function from_todo_story_to_quake_calendar(todos, storys) {
  let results = [];
  for (let todo of todos) {
    results.push({
      type: "todo",
      title: todo.title,
      content: todo.content,
      created_date: todo.created_date,
      updated_date: todo.updated_date
    })
  }

  for (let story of storys) {
    results.push({
      type: "story",
      title: story.title,
      content: story.content,
      created_date: story.created_date,
      updated_date: story.updated_date
    })
  }
  return results;
}

const tl_show_calendar = async (context, commands) => {
  const el = document.createElement('quake-calendar');

  let todos = await Quake.query('todo', '', {
    filter: 'created_date > 1609257600 AND updated_date < 1640793600'
  });

  let storys = await Quake.query('story', '', {
    filter: 'created_date > 1609257600 AND updated_date < 1640793600'
  });

  let data = from_todo_story_to_quake_calendar(todos, storys);
  el.setAttribute('data', JSON.stringify(data));

  return el;
}

Quake.transflow.add({name: 'show_calendar', action: tl_show_calendar, display: "Show calendar"})
Quake.flows['tl_show_calendar'] = tl_show_calendar

function from_story_to_quake_timeline(storys) {
  let results = [];
  for (let story of storys) {
    results.push({
      type: "story",
      title: story.title,
      content: story.content,
      date: story.created_date
    })
  }
  return results;
}

const tl_show_timeline = async (context, commands) => {
  const el = document.createElement('quake-timeline');

  let storys = await Quake.query('story', '', {
    filter: 'created_date > 1609257600 AND updated_date < 1640793600'
  });

  let data = from_story_to_quake_timeline(storys);
  el.setAttribute('data', JSON.stringify(data));

  return el;
}

Quake.transflow.add({name: 'show_timeline', action: tl_show_timeline, display: "Show calendar"})
Quake.flows['tl_show_timeline'] = tl_show_timeline

function from_blog_to_quake_calendar(storys) {
  let results = [];
  for (let story of storys) {
    results.push({
      type: "story",
      title: story.title,
      content: story.content,
      created_date: story.created_date,
      updated_date: story.updated_date
    })
  }
  results = results.concat(storys);
  return results;
}

const tl_show_blog_calendar = async (context, commands) => {
  const el = document.createElement('quake-calendar');

  let storys = await Quake.query('story', '', {
    filter: 'created_date > 1609257600 AND updated_date < 1640793600'
  });

  let data = from_blog_to_quake_calendar(storys);
  el.setAttribute('data', JSON.stringify(data));

  return el;
}

Quake.transflow.add({name: 'show_blog_calendar', action: tl_show_blog_calendar, display: "Show calendar"})
Quake.flows['tl_show_blog_calendar'] = tl_show_blog_calendar
