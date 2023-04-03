
console.log("connected");
getCurrentDirectory();

var button = document.getElementById('fsubmit');
var input = document.getElementById('fentry');
var dinput = document.getElementById('dentry');
var dbutton = document.getElementById('dsubmit');
var searchResults = document.getElementById("search-results");
var loading = document.getElementById("loading");

let query = "";
let dir= ""

input.addEventListener("keyup", (ev) => {
    query = ev.target.value
    // console.log(query);
    // sendQuery(query);
})

dinput.addEventListener("keyup", (ev) => {
    dir = ev.target.value
    // console.log(dir);
    // sendDir(dir);
})

button.onclick = function() { 
    sendQuery(query);
    // console.log(JSON.stringify(query)); 
}

dbutton.onclick = function() { 
    sendDir(dir);
    // console.log(JSON.stringify(dir)); 
}

function fillDirectoryInput(dir) {
    // console.log(dir.message)
    dinput.value = dir.message;
}

function fillLoading(){
    let inner = document.createTextNode("Hashing Directory...");
    loading.appendChild(inner);
}

function RemoveLoading(){
    while( loading.firstChild ){
        loading.removeChild( loading.firstChild );
      }
}

const delay = ms => new Promise(res => setTimeout(res, ms));

function getDoc(doc) {
    window.open("http://127.0.0.1:8080/?doc="+doc, '_blank').focus();
    // window.location.replace("http://127.0.0.1:8080/?doc="+doc);
};

async function wait() {
    // await delay(2000)
    fillLoading();
    fetch("/", {
        method: "GET",
        mode: 'cors',
        cache: 'no-cache',
        credentials: 'same-origin',
        redirect:"follow",
        referrerPolicy : "no-referrer",
        headers: {"Content-Type": "text/html"},
    })
        .then(res => { console.log(res); RemoveLoading(); return; });
}

function getCurrentDirectory() {
    fetch("/api/current-dir", {
        method: "GET",
        mode: 'cors',
        cache: 'no-cache',
        credentials: 'same-origin',
        redirect:"follow",
        referrerPolicy : "no-referrer",
        headers: {"Content-Type": "application/json"},
    })
    .then(res => { 
        res.json()
        .then(json => fillDirectoryInput(json))});
}

function sendDir(dir) {
    const dirObj = {message: dir};
    fetch("/api/change-dir", {
        method: "POST",
        mode: 'cors',
        cache: 'no-cache',
        credentials: 'same-origin',
        redirect:"follow",
        referrerPolicy : "no-referrer",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify(dirObj)
    })
      .then(res => { 
        wait() 
    });
};


function sendQuery(query) {
    const queryObj = {message: query};
    fetch("/api/query", {
        method: "POST",
        mode: 'cors',
        cache: 'no-cache',
        credentials: 'same-origin',
        redirect:"follow",
        referrerPolicy : "no-referrer",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify(queryObj)
    })
      .then(res => {
        res.json()
        .then(json => process_query_response(json))});
};

let totalPaths = [];
let totalLiElements = [];



function process_query_response(json){
    while( searchResults.firstChild ){
        searchResults.removeChild( searchResults.firstChild );
      }
    totalPaths=[];
    for(let i = 0; i < json.paths.length; i++){
        add_path(json.paths[i]);
    }

    totalLiElements = searchResults.children;
    for(let i = 0; i < totalLiElements.length; i++){
        totalLiElements[i].addEventListener( 'click', ()=>{
            let parts = totalPaths[i].base + totalPaths[i].main + totalPaths[i].extension;
            getDoc(parts);
        })
    }

}

function trimPath(path){
    let start = getStart(path);
    let extensionIndex = getExtension(path);
    totalPaths.push({
        base: path.substring(0, start), 
        main : path.substring(start, extensionIndex), 
        extension: path.substring(extensionIndex, path.length)});
    return totalPaths[totalPaths.length - 1].main;
}

function getStart(path) {
    for (let i = path.length-1; i >= 0; i--){
        if(path[i] === "/") return i+1;
    }
    return 0;
}

function getExtension(path) {
    for (let i = path.length-1; i >= 0; i--){
        if(path[i] === ".") return i;
    }
    return path.length;
}

function add_path(path) {
    var li = document.createElement("li");
    let inner = document.createTextNode(trimPath(path));
    li.appendChild(inner);
    searchResults.appendChild(li);
  }
  
