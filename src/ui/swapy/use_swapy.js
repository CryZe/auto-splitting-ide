const container = document.getElementById(await dioxus.recv());

const swapy = Swapy.createSwapy(container, {
  animation: "dynamic",
});

swapy.enable(true);
