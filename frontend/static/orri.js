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

    function onLoad(elem, callback) {
        if (!elem.files || elem.files.length === 0) {
            callback(null);
            return;
        }

        const file = elem.files[0];
        const reader = new FileReader();

        reader.onload = function(e) {
            callback({
                name: file.name,
                size: file.size,
                dataUrl: e.target.result,
            });
        };

        reader.readAsDataURL(file);
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
    };
}

function Crypto() {
    function randomString(length) {
        var randomNumbers = new Uint8Array(length);
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
