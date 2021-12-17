const Router = Vaadin.Router;
const outlet = document.getElementById('outlet');
const router = new Router(outlet);

const Quake = {
  // init flow map
  flows: {},
  client: new MeiliSearch({
    host: 'http://127.0.0.1:7700'
  }),
  router: router,
  transflow: {
    mapping: {},
    add: function (route) {
      let conf = {
        path: `/transflow/custom/${route.name}`,
        action: route.action
      };

      Quake.router.addRoutes(conf);

      const nav = document.createElement('a');
      nav.setAttribute("href", conf.path);
      nav.innerText = route.name;

      let navNode = document.getElementById("transflow-nav");
      navNode.appendChild(nav);
    }
  },
  query: function (entry, query, filter) {
    let index = Quake.client.index(entry);
    let options = {
      limit: 99,
      attributesToHighlight: ['overview']
    };

    // check filter
    Object.assign(options, filter);

    return index.search(query, options).then((result) => {
      return result.hits
    })
  }
}

window.Quake = Quake;

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

  let url = `/entry/${params.type}?text=${text}`
  const response = await fetch(url, {
    method: 'POST'
  });

  const entry = await response.json();
  params.id = entry.id;

  return create_editor_element(entry, params);
}

const update_entry = async (entry_type, id, properties) => {
  let response = await fetch(`/entry/${entry_type}/${id}`, {
    method: 'POST',
    body: JSON.stringify({properties: properties})
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

  editor.addEventListener("clickPageLink", function (event) {
    let data = event.detail;
    console.log(data);
    Router.go(`/show/${data.type}/${data.id}`);
  });

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
