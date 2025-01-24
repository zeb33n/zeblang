export function zeblang_print(s){
  var stdout = document.getElementById("stdout"); 
  stdout.innerHTML += "<div>"
  stdout.innerHTML += s;
  stdout.innerHTML += "</div>";
}
