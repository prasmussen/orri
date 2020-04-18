function Form() {

    function getData(elem) {
        const data = new FormData(elem);
        const result = {};

        data.forEach((value, key) => {
            result[key] = value;
        });

        return result;
    }

    function onSubmit(form, callback) {
        form.addEventListener('submit', event => {
            event.preventDefault();

            const data = getData(form);
            callback(data);
        });

    }

    return {
        onSubmit: onSubmit,
    };
}

function File() {

    function onLoad(elem) {
        if (!elem.files || elem.files.length === 0) {
            return Promise.reject("No file selected");
        }

        const file = elem.files[0];
        const reader = new FileReader();

        const promise = new Promise((resolve, reject) => {
            reader.onload = (e) => {
                resolve({
                    name: file.name,
                    size: file.size,
                    dataUrl: e.target.result,
                });
            };

            reader.onerror = (e) => {
                reject(e);
            };
        });

        reader.readAsDataURL(file);

        return promise;
    }

    return {
        onLoad: onLoad,
    };
}

function Api() {
    function post(url, data) {
        return request(url, "POST", data);
    }

    function put(url, data) {
        return request(url, "PUT", data);
    }

    function rejectErrors(res) {
        if (res.ok) {
            return res;
        }

        return Promise.reject(res);
    }

    function request(url, method, data) {
        return fetch(url, {
            method: method,
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify(data)
        });
    }

    return {
        post: post,
        put: put,
        rejectErrors: rejectErrors,
    };
}

function Crypto() {
    function randomString(length) {
        const randomNumbers = new Uint8Array(length);
        crypto.getRandomValues(randomNumbers);

        return Array.from(randomNumbers)
            .map(x => x.toString(16))
            .join("")
            .slice(0, length);
    }

    return {
        randomString: randomString,
    };
}

function Page() {

    function showAlert(elem, msg) {
        elem.classList.remove("display-none");
        elem.innerText = msg;
    }

    return {
        showAlert: showAlert,
    };
}

function ErrorMessage() {

    function prepare(err) {
        if (typeof err === "string") {
            return Promise.resolve(err);
        }

        if (typeof err === "object" && err.json) {
            return err.json().then(json => {
                if (json && typeof json.error === "string") {
                    return json.error;
                }

                return "Something went wrong";
            });
        }

        return "Something went wrong";
    }

    return {
        prepare: prepare,
    }
}
