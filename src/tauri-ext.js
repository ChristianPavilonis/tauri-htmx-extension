const { invoke } = window.__TAURI__.core;
const { listen, emit, emitTo } = window.__TAURI__.event;

/** @type {import("./htmx").HtmxInternalApi} */
let api;

window.addEventListener("DOMContentLoaded", () => {
  htmx.defineExtension("tauri", {
    init(apiRef) {
      api = apiRef;
    },
    onEvent: (name, evt) => {
      let parent = evt.target || evt.detail.elt;

      switch (name) {
        case "htmx:load":
          registerInvokeTiggers(parent);
          registerTauriListeners(parent);
          registerTauriEmitters(parent);
          break;
      }
    },
  });
});

function registerInvokeTiggers(parent) {
  parent.querySelectorAll("[tauri-invoke]").forEach((el) => {
    let nodeData = api.getInternalData(el);
    let triggerSpecs = api.getTriggerSpecs(el);
    console.log(triggerSpecs);

    triggerSpecs.forEach((ts) => {
      // since we're already in the load event we call it now
      if (ts.trigger === "load") {
        callInvoke(el);
      }
      api.addTriggerHandler(el, ts, nodeData, () => {
        callInvoke(el);
      });
    });
  });
}

function callInvoke(el) {
  let handle = api.getAttributeValue(el, "tauri-invoke");
  let settleInfo = api.makeSettleInfo(el);
  let swapSpec = api.getSwapSpecification(el);
  let target = api.getTarget(el);
  let input = api.getInputValues(el);

  invoke(handle, input.values).then((response) => {
    if (response) {
      api.swap(target, response, swapSpec);
    }

    api.settleImmediately(settleInfo.tasks);
  });
}

function registerTauriListeners(parent) {
  parent.querySelectorAll("[tauri-listen]").forEach((el) => {
    let eventName = api.getAttributeValue(el, "tauri-listen");
    listen(eventName, (event) => {
      let payload = event.payload;
      let settleInfo = api.makeSettleInfo(el);
      let swapSpec = api.getSwapSpecification(el);
      let target = api.getTarget(el);

      if (payload) {
        api.swap(target, payload, swapSpec);
      }

      api.settleImmediately(settleInfo.tasks);
    });
  });
}

function registerTauriEmitters(parent) {
  parent.querySelectorAll("[tauri-emit]").forEach((el) => {
    let nodeData = api.getInternalData(el);
    let triggerSpecs = api.getTriggerSpecs(el);

    triggerSpecs.forEach((ts) => {
      api.addTriggerHandler(el, ts, nodeData, () => {
        callEmit(el);
      });
    });
  });
}

function callEmit(el) {
  let eventName = api.getAttributeValue(el, "tauri-emit");
  let input = api.getInputValues(el);

  emit(eventName, input.values);
}
