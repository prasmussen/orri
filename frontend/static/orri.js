const orri = {
    form: Form(),
    button: Button(),
    file: File(),
    api: Api(),
    crypto: Crypto(),
    page: Page(),
};

function Form() {

    function getData(elem) {
        const data = new FormData(elem);
        const result = {};

        data.forEach((value, key) => {
            result[key] = value;
        });

        return result;
    }

    function onSubmit(form, submitButton, callback) {
        form.addEventListener('submit', event => {
            event.preventDefault();

            const data = getData(form);

            if (!submitButton.disabled) {
                submitButton.disabled = true;

                callback(data, () => {
                    submitButton.disabled = false;
                });
            }
        });
    }

    return {
        onSubmit: onSubmit,
    };
}

function Button() {

    function prepareFieldName(name) {
        const nameWithoutPrefix = name.replace(/^apiBody/, "");

        return [
            nameWithoutPrefix.charAt(0).toLowerCase(),
            nameWithoutPrefix.slice(1),
        ].join("");
    }

    function getBodyData(button) {
        const buttonData = Object.assign({}, button.dataset);
        const result = {};

        for (let [key, value] of Object.entries(buttonData)) {
            if (!key.startsWith("apiBody")) {
                continue
            }

            const fieldName = prepareFieldName(key);

            if (fieldName.length === 0) {
                continue;
            }


            result[fieldName] = value;
        }

        return result;
    }

    function onClick(button, callback) {
        button.addEventListener('click', () => {
            const data = getBodyData(button);

            if (!button.disabled) {
                button.disabled = true;

                callback(data, () => {
                    button.disabled = false;
                });
            }
        });
    }

    return {
        onClick: onClick,
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
    function request(method, url, data) {
        return fetch(url, {
            method: method,
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify(data)
        });
    }

    function rejectErrors(res) {
        if (res.ok) {
            return res;
        }

        return Promise.reject(res);
    }

    return {
        request: request,
        rejectErrors: rejectErrors,
    };
}

function Crypto() {
    function randomString(length) {
        const randomNumbers = new Uint32Array(length);
        crypto.getRandomValues(randomNumbers);

        return Array.from(randomNumbers)
            .map(x => x.toString(36))
            .join("")
            .slice(0, length);
    }

    return {
        randomString: randomString,
    };
}

function Page() {

    function getElement(selector) {
        const elem = document.querySelector(selector);
        if (!elem) {
            throw new Error("Failed to find element: " + selector);
        }

        return elem;
    }

    function hideElement(elem) {
        elem.classList.add("display-none");
    }

    function unhideElement(elem) {
        elem.classList.remove("display-none");
    }

    function showError(errorAlert, err) {
        return formatError(err).then(msg => {
            showAlert(errorAlert, msg);
        });
    }

    function showAlert(elem, msg) {
        unhideElement(elem);
        elem.innerText = msg;
    }

    function formatError(err) {
        if (typeof err === "string") {
            return Promise.resolve(err);
        }

        if (typeof err === "object" && err.json) {
            return err.json().then(json => {
                if (json && typeof json.error === "string") {
                    return json.error;
                }

                console.log("Unhandled error in response", err, json);
                return Promise.resolve("Something went wrong: " + err.statusText);
            }).catch(data => {
                console.log("Failed reading json from response", err, data);
                return Promise.resolve("Something went wrong: " + err.statusText);
            }) ;
        }

        if (typeof err === "object" && err.message) {
            return Promise.resolve(err.message);
        }

        console.log("Unhandled error", err);
        return Promise.resolve("Something went wrong");
    }

    return {
        getElement: getElement,
        hideElement: hideElement,
        unhideElement: unhideElement,
        showError: showError,
    };
}
