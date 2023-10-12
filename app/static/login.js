document.addEventListener('DOMContentLoaded', () => {

  let loginBtn = document.getElementById('login-btn')
  loginBtn.addEventListener("click", login, false)

  function login() {

    let usernameHelper = document.getElementById('username-help')
    let passwordHelper = document.getElementById('password-help')

    let username = document.getElementById('username-field').value
    let password = document.getElementById('password-field').value

    if (username.length == 0) {
      usernameHelper.style.visibility = ""
      return
    } else {
      usernameHelper.style.visibility = "hidden"
    }

    if (password.length == 0) {
      passwordHelper.style.visibility = ""
      return
    }

    passwordHelper.style.visibility = "hidden"

    let userInfo = {
      username: username,
      password: password
    }

    let userInfoJson = JSON.stringify(userInfo)

    loginRequest(userInfoJson)
  }

  function loginRequest(body) {

    var xhr = new XMLHttpRequest();

    var url = '/api/user_login';

    xhr.open('POST', url, true);
    xhr.setRequestHeader('Content-Type', 'application/json');

    xhr.onreadystatechange = function() {
      if (xhr.readyState === XMLHttpRequest.DONE) {
        if (xhr.status === 200) {

          location.assign('/')
        }
        // TODO handle error
      }
    };

    xhr.send(body);
  }

})
