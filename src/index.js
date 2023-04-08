
console.log("connected");
getCurrentDirectory();
getAllDriectories();
sendQuery("");

var button = document.getElementById('fsubmit');
var input = document.getElementById('fentry');
var dinput = document.getElementById('dentry');
var dbutton = document.getElementById('dsubmit');
var searchResults = document.getElementById("search-results");
var loading = document.getElementById("loading");
var myDropdown = document.getElementById("myDropdown");
var checkBox = document.getElementById("showPath");

let query = "";
let dir= ""



input.addEventListener("keyup", (ev) => {
    query = ev.target.value
})

dinput.addEventListener("keyup", (ev) => {
    dir = ev.target.value
})

checkBox.addEventListener("click", (ev) => {
    sendQuery(query);
})

button.onclick = function() { 
    sendQuery(query);
}

dbutton.onclick = function() { 
    sendDir(dir);
}



let directories = [];





function fillDirectoryInput(dir) {
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
      sendQuery(query);
}

function fillDirectoryList(json){
    for(let i = 0; i < json.paths.length; i++){
        directories.push(json.paths[i]);
        var p = document.createElement("p");
        let inner = document.createTextNode(directories[i]);
        p.appendChild(inner);
        myDropdown.appendChild(p);
        p.addEventListener("click", () => {
            sendDir(inner.textContent);
            dinput.value = inner.textContent;
        })
        
    }
}

function fillDirectoryDropdown(){
    myDropdown.classList.toggle("show");
}

window.onclick = function(event) {
    if (event.target !== dinput && event.target !== myDropdown) {
      if(myDropdown.classList.contains("show")){
        myDropdown.classList.remove("show");
      }
    }
}
    





function getAllDriectories() {
    fetch("/api/all-dirs", {
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
        .then(json => fillDirectoryList(json))});
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

function getDoc(doc) {
    window.open("http://127.0.0.1:8080/?doc="+doc, '_blank').focus();
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
        .then(res => { RemoveLoading(); return; });
}

const delay = ms => new Promise(res => setTimeout(res, ms));

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


function trimPath(path, trim){
    let start = getStart(path);
    let extensionIndex = getExtension(path);
    totalPaths.push({
        base: path.substring(0, start), 
        main : path.substring(start, extensionIndex), 
        extension: path.substring(extensionIndex, path.length)});
    if(trim)
        return totalPaths[totalPaths.length - 1].main;
    else
        return totalPaths[totalPaths.length - 1].base + totalPaths[totalPaths.length - 1].main +
        totalPaths[totalPaths.length - 1].extension;
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
    let inner = document.createTextNode(trimPath(path, !checkBox.checked));
    li.appendChild(inner);
    searchResults.appendChild(li);
  }
  
