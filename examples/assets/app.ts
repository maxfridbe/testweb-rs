
interface IPerson {
    firstname: string;
    lastname:  string;
}

function doPost<T,Tresp>(url, data:T):Promise<Tresp> {
  return new Promise<Tresp>((resolve,reject)=>{
    var xmlDoc = new XMLHttpRequest();

    xmlDoc.open('POST', url, true);
    xmlDoc.setRequestHeader("Content-type", "application/json");

    xmlDoc.onreadystatechange = function() {
      if (xmlDoc.readyState === 4 && xmlDoc.status === 200) {
        var resp:Tresp = JSON.parse(xmlDoc.response);
        resolve(resp);
      }
    }

    xmlDoc.send(JSON.stringify(data));
  });
}

const $app = document.getElementById("app");
$app.innerHTML = `
  <h1>Hello Rust</h1>
`;

doPost("/a/post/request",<IPerson>{firstname:"bob",lastname:"jone"}).then((s)=>{
  console.log(s);
});
