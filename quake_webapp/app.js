const Router = Vaadin.Router;
const outlet = document.getElementById('outlet');
const router = new Router(outlet);

const Quake = {
  client: new MeiliSearch({
    host: 'http://127.0.0.1:7700'
  }),
  router: router,
  query: function (entry, query, filter) {
    let index = Quake.client.index(entry);
    let options = {
      limit: 40,
      attributesToHighlight: ['overview']
    };

    // check filter
    Object.assign(options, filter);

    return index.search(query, options).then((result) => {
      return result.hits
    })
  }
}

const home = (context, commands) => {
  const dashboard = document.createElement('quake-dashboard');
  dashboard.addEventListener("dispatchAction", function (e) {
    let define = e.detail;
    handleAction(define);
  });

  return dashboard
};

function create_editor_element(entry, params) {
  const editor = document.createElement('quake-editor');

  // todo: should set value first,?
  editor.setAttribute('value', entry.content);

  editor.setAttribute('id', entry.id);
  editor.setAttribute('title', entry.title);
  editor.addEventListener("onSave", function (event) {
    let data = event.detail;
    update_entry(params.type, params.id, {
      title: data.title,
      content: data.value.replaceAll("\\\n", "\n")
    })
  });
  return editor
}

const edit_entry = async (context, commands) => {
  let params = context.params;
  let response = await fetch(`/entry/${params.type}/${params.id}`)
  const entry = await response.json();

  return create_editor_element(entry, params);
}

const create_entry = async (context, commands) => {
  let params = context.params;
  let url_params = new URLSearchParams(context.search);
  let text = url_params.get('text');

  let url = `/entry/${params.type}/new?text=${text}`
  const response = await fetch(url, {
    method: 'POST'
  });

  const entry = await response.json();
  params.id = entry.id;

  return create_editor_element(entry, params);
}

const tl_timeline = async (context, commands) => {
  const el = document.createElement('quake-calendar');

  let todos = await Quake.query('todo');
  let blogs = await Quake.query('blog');
  let data = from_todo_blog_to_quake_calendar(todos, blogs);

  el.setAttribute('data', JSON.stringify(data));

  return el;
}

const update_entry = async (entry_type, id, fields) => {
  let response = await fetch(`/entry/${entry_type}/${id}`, {
    method: 'POST',
    body: JSON.stringify({fields: fields})
  });

  let data = await response.json();
  console.log(data);
}

const show_entry = async (context, commands) => {
  let params = context.params;
  let response = await fetch(`/entry/${params.type}/${params.id}`)
  const entry = await response.json();

  const editor = document.createElement('quake-render');
  editor.setAttribute('content', entry.content);

  return editor
}

function handleAction(define) {
  if (define.action === 'add') {
    Router.go(`/entry/${define.object}/new?text=${define.text}`)
  } else if(define.action === 'show') {
    Router.go(`/show/${define.object}/${define.parameters[0]}`);
  } else if (define.parameters.length > 0) {
    Router.go(`/edit/${define.object}/${define.parameters[0]}`);
  } else {
    console.log("some action");
  }
}

router.setRoutes([
  {path: '/', action: home},
  {path: '/entry/:type/new', action: create_entry},
  {path: '/edit/:type/:id', action: edit_entry},
  {path: '/show/:type/:id', action: show_entry},
]);

// move to auto generate in transflow
Quake.router.addRoutes({path: '/transflow/show_calendar', action: tl_timeline},)
