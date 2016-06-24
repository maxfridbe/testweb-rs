function doPost(url, data) {
    return new Promise(function (resolve, reject) {
        var xmlDoc = new XMLHttpRequest();
        xmlDoc.open('POST', url, true);
        xmlDoc.setRequestHeader("Content-type", "application/json");
        xmlDoc.onreadystatechange = function () {
            if (xmlDoc.readyState === 4 && xmlDoc.status === 200) {
                var resp = JSON.parse(xmlDoc.response);
                resolve(resp);
            }
        };
        xmlDoc.send(JSON.stringify(data));
    });
}
var $app = document.getElementById("app");
$app.innerHTML = "\n  <h1>Hello Rust</h1>\n";
doPost("/a/post/request", { firstname: "bob", lastname: "jone" }).then(function (s) {
    console.log(s);
});
