const {invoke} = window.__TAURI__.tauri
let shortcut = window.__TAURI__.globalShortcut
let wd = window.__TAURI__.window.getCurrent();

// 默认隐藏
// wd.hide();

// $(window).on('blur', function() {
//     invoke('toggle_window', {})
//         .then((response) => {
//             console.log(response);
//         })
// });

$("#titlebar-clear").on("click", function (event) {
    invoke('clear', {})
        .then((response) => {
            console.log(response);
        })
});

$("#titlebar-close").on("click", function (event) {
    invoke('toggle_window', {})
        .then((response) => {
            console.log(response);
        })
});

const shortcutId = shortcut.register('Ctrl+Alt+G', () => {
    invoke('toggle_window', {})
        .then((response) => {
            console.log(response);
        })
});

// 检查是否成功注册
if (shortcutId) {
    console.log('Global shortcut registered with ID:', shortcutId);
} else {
    console.error('Failed to register global shortcut');
}

$("#content").on("click", ".item", function (event) {
    invoke('set_data', {"data": $(this).text()})
        .then((response) => {
            if (response === "OK") {
                layer.msg('Copied!', {time: 1000});
            } else {
                alert("发生了一些错误，请忽略");
            }
        })
});

function init() {
    invoke('items', {s: ""})
        .then((response) => {
            let first = $("#content .item:first").text();
            if (first === response[0]) {
                console.log("not change");
                return;
            }
            $("#content").empty();
            for (let i in response) {
                let tmp = $('<div class="form-control mb-3 shadow-sm overflow-hidden item"></div>');
                tmp.text(response[i]);
                $("#content").append(tmp);
            }
        })
}

setInterval(function () {
    init();
}, 2000);