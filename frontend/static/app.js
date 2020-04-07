
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
        return fetch(url, {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify(data)
        });
    }

    return {
        post: post,
    };
}

(function() {

    Form().onSubmit(document.getElementById("document"), data => {
        File().onLoad(document.getElementById("file"), file => {
            if (!file) {
                console.log("Empty file");
                return;
            }

            data.dataUrl = file.dataUrl;

            Api().post("/api/sites", data).then(res => {
                console.log(res);
            });
        });

    });

})();
