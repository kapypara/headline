document.addEventListener('DOMContentLoaded', () => {

  let forgetBtn = document.getElementById('forget-password-btn')
  forgetBtn.addEventListener("click", forgetMsg, false);

  function forgetMsg() {

    if (document.getElementById('check-with-me') != null) {
      return
    }

    const msgString = "Oh I don't know, check with me and I'll do something"
    let msg = document.createTextNode(msgString)


    let closeBtn = document.createElement('button')
    closeBtn.className = "delete"

    closeBtn.addEventListener("click", () => {

      requestAnimationFrame(() =>
        setTimeout(() => {
          msgBox.style.opacity = "0";
        })
      )

      setTimeout( () => {
        msgBox.remove()
        // document.getElementById('check-with-me').remove()
      }, 1e3)

    }, false)


    let msgBox = document.createElement('div')
    msgBox.className = "notification is-warning mb-0 is-two-fifth"
    msgBox.id = "check-with-me"


    msgBox.appendChild(closeBtn)
    msgBox.appendChild(msg)


    msgBox.style.transition = "0.2s";
    msgBox.style.opacity = "0.0";

    document.getElementById('login-box').appendChild(msgBox)

    requestAnimationFrame(() =>
      setTimeout(() => {
        msgBox.style.opacity = "1";
      })
    )
  }

})
