/* Code modified from the blender website
 * https://www.blender.org/wp-content/themes/bthree/assets/js/get_os.js?x82196
 */

let options = {
    windows64: "x86_64-pc-windows",
    windows32: "i686-pc-windows",
    windowsArm: "aarch64-pc-windows",

    mac64: "x86_64-apple",
    mac32: "i686-apple",
    macSilicon: "aarch64-apple",

    linux64: "x86_64-unknown-linux",
    linux32: "i686-unknown-linux",
    linuxArm: "aarch64-unknown-linux",

    // ios: "ios",
    // android: "linux-android",
    // freebsd: "freebsd",
};

function isAppleSilicon() {
    try {
        var glcontext = document.createElement("canvas").getContext("webgl");
        var debugrenderer = glcontext
            ? glcontext.getExtension("WEBGL_debug_renderer_info")
            : null;
        var renderername =
            (debugrenderer &&
                glcontext.getParameter(debugrenderer.UNMASKED_RENDERER_WEBGL)) ||
            "";
        if (renderername.match(/Apple M/) || renderername.match(/Apple GPU/)) {
            return true;
        }

        return false;
    } catch (e) {}
}

function getOS() {
    var OS = options.windows64.default;
    var userAgent = navigator.userAgent;
    var platform = navigator.platform;

    if (navigator.appVersion.includes("Win")) {
        if (
            !userAgent.includes("Windows NT 5.0") &&
            !userAgent.includes("Windows NT 5.1") &&
            (userAgent.indexOf("Win64") > -1 ||
                platform == "Win64" ||
                userAgent.indexOf("x86_64") > -1 ||
                userAgent.indexOf("x86_64") > -1 ||
                userAgent.indexOf("amd64") > -1 ||
                userAgent.indexOf("AMD64") > -1 ||
                userAgent.indexOf("WOW64") > -1)
        ) {
            OS = options.windows64;
        } else {
            if (
                window.external &&
                window.external.getHostEnvironmentValue &&
                window.external
                    .getHostEnvironmentValue("os-architecture")
                    .includes("ARM64")
            ) {
                OS = options.windowsArm;
            } else {
                try {
                    var canvas = document.createElement("canvas");
                    var gl = canvas.getContext("webgl");

                    var debugInfo = gl.getExtension("WEBGL_debug_renderer_info");
                    var renderer = gl.getParameter(debugInfo.UNMASKED_RENDERER_WEBGL);
                    if (renderer.includes("Qualcomm")) OS = options.windowsArm;
                } catch (e) {}
            }
        }
    }

    //MacOS, MacOS X, macOS
    if (navigator.appVersion.includes("Mac")) {
        if (
            navigator.userAgent.includes("OS X 10.5") ||
            navigator.userAgent.includes("OS X 10.6")
        ) {
            OS = options.mac32;
        } else {
            OS = options.mac64;

            const isSilicon = isAppleSilicon();
            if (isSilicon) {
                OS = options.macSilicon;
            }
        }
    }

    // linux
    if (platform.includes("Linux")) {
        OS = options.linux64;
        // FIXME: Can we find out whether linux 32-bit or ARM are used?
    }

    // if (
    //     userAgent.includes("iPad") ||
    //     userAgent.includes("iPhone") ||
    //     userAgent.includes("iPod")
    // ) {
    //     OS = options.ios;
    // }
    // if (platform.toLocaleLowerCase().includes("freebsd")) {
    //     OS = options.freebsd;
    // }

    return OS;
}

let os = getOS();
window.os = os;

// Unhide and hydrate selector with events
const archSelect = document.querySelector(".arch-select");
if (archSelect) {
    archSelect.classList.remove("hidden");
    const selector = document.querySelector("#install-arch-select");
    if (selector) {
        selector.addEventListener("change", onArchChange);
    }
}

// Hydrate tab buttons with events
Array.from(document.querySelectorAll(".install-tab[data-id]")).forEach((tab) => {
    tab.addEventListener("click", onTabClick);
});

function onArchChange(evt) {
    // Get target
    const target = evt.currentTarget.value;
    // Find corresponding installer lists
    const newContentEl = document.querySelector(`.arch[data-arch=${target}]`);
    const oldContentEl = document.querySelector(`.arch[data-arch]:not(.hidden)`);
    // Hide old content element (if applicable)
    if (oldContentEl) {
        oldContentEl.classList.add("hidden");
    }
    // Show new content element
    newContentEl.classList.remove("hidden");
    // Show the first tab's content if nothing was selected before
    if (newContentEl.querySelectorAll(".install-tab.selected").length === 0) {
        const firstContentChild = newContentEl.querySelector(".install-content:first-of-type");
        const firstTabChild = newContentEl.querySelector(".install-tab:first-of-type");
        firstContentChild.classList.remove("hidden");
        if (firstTabChild) {
            firstTabChild.classList.add("selected");
        }
    }
    // Hide "no OS detected" message
    const noDetectEl = document.querySelector(".no-autodetect");
    noDetectEl.classList.add("hidden");
    // Hide Mac hint
    document.querySelector(".mac-switch").classList.add("hidden");
}

function onTabClick(evt) {
    // Get target and ID
    const {triple, id} = evt.currentTarget.dataset;
    if (triple) {
        // Find corresponding content elements
        const newContentEl = document.querySelector(`.install-content[data-id="${String(id)}"][data-triple=${triple}]`);
        const oldContentEl = document.querySelector(`.install-content[data-triple=${triple}][data-id]:not(.hidden)`);
        // Find old tab to unselect
        const oldTabEl = document.querySelector(`.install-tab[data-triple=${triple}].selected`);
        // Hide old content element
        if (oldContentEl && oldTabEl) {
            oldContentEl.classList.add("hidden");
            oldTabEl.classList.remove("selected");
        }

        // Unhide new content element
        newContentEl.classList.remove("hidden");
        // Select new tab element
        evt.currentTarget.classList.add("selected");
    }
}

const allPlatforms = Array.from(document.querySelectorAll(`.arch[data-arch]`));
let hit = allPlatforms.find(
    (a) => {
        // Show Intel Mac downloads if no M1 Mac downloads are available
        if (
            a.attributes["data-arch"].value.includes(options.mac64) &&
            os.includes(options.macSilicon) &&
            !allPlatforms.find(p => p.attributes["data-arch"].value.includes(options.macSilicon))) {
            // Unhide hint
            document.querySelector(".mac-switch").classList.remove("hidden");
            return true;
        }
        return a.attributes["data-arch"].value.includes(os);
    }
);

if (hit) {
    hit.classList.remove("hidden");
    const selectEl = document.querySelector("#install-arch-select");
    selectEl.value = hit.dataset.arch;
    const firstContentChild = hit.querySelector(".install-content:first-of-type");
    const firstTabChild = hit.querySelector(".install-tab:first-of-type");
    firstContentChild.classList.remove("hidden");
    if (firstTabChild) {
        firstTabChild.classList.add("selected");
    }
} else {
    const noDetectEl = document.querySelector(".no-autodetect");
    if (noDetectEl) {
        const noDetectElDetails = document.querySelector(".no-autodetect-details");
        if (noDetectElDetails) {
            noDetectElDetails.innerHTML = `We detected you're on ${os} but there don't seem to be installers for that. `
        }
        noDetectEl.classList.remove("hidden");
    }
}

let copyButtons = Array.from(document.querySelectorAll("[data-copy]"));
if (copyButtons.length) {
    copyButtons.forEach(function (element) {
        element.addEventListener("click", () => {
            navigator.clipboard.writeText(element.attributes["data-copy"].value);
        });
    });
}

// Toggle for pre releases
const checkbox = document.getElementById("show-prereleases");

if (checkbox) {
    checkbox.addEventListener("click", () => {
        const all = document.getElementsByClassName("pre-release");

        if (all) {
            for (var item of all) {
                item.classList.toggle("hidden");
            }
        }
    });
}