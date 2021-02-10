addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
    const { do_hmac } = wasm_bindgen;
    await wasm_bindgen(wasm)
    let payload = await getParameterByName("payload", request) || "Hello world!"
    const hmac = do_hmac(payload)
    return new Response( JSON.stringify({payload, hmac}), {status: 200})
}




async function getParameterByName(name, request) {
  const url = request.url
  name = name.replace(/[\[\]]/g, '\\$&')
  name = name.replace(/\//g, '')
  var regex = new RegExp('[?&]' + name + '(=([^&#]*)|&|#|$)'),
    results = regex.exec(url)

  if (!results) return null
  else if (!results[2]) return ''
  else if (results[2]) {
    results[2] = results[2].replace(/\//g, '')
  }
  
  return decodeURIComponent(results[2].replace(/\+/g, ' '));
}
