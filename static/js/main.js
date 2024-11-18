  import init, { HdrProcessor } from "./pkg/hdr_web_app.js";

  let hdrProcessor;
  let uploadedFiles = [];

  async function initialize() {
      await init();
      hdrProcessor = new HdrProcessor();
      setupUI();
  }

  function setupUI() {
      const dropzone = document.getElementById("dropzone");
      const fileInput = document.getElementById("fileInput");
      const processButton = document.getElementById("process");

      dropzone.addEventListener("click", () => fileInput.click());
      dropzone.addEventListener("dragover", (e) => {
          e.preventDefault();
          dropzone.classList.add("dragover");
      });
      dropzone.addEventListener("dragleave", () => {
          dropzone.classList.remove("dragover");
      });
      dropzone.addEventListener("drop", handleDrop);
      fileInput.addEventListener("change", handleFileSelect);
      processButton.addEventListener("click", processHDR);
  }

  async function handleDrop(e) {
      e.preventDefault();
      const dropzone = document.getElementById("dropzone");
      dropzone.classList.remove("dragover");

      const files = [...e.dataTransfer.files].filter((f) =>
          f.type.startsWith("image/"),
      );
      await handleFiles(files);
  }

  async function handleFileSelect(e) {
      const files = [...e.target.files].filter((f) =>
          f.type.startsWith("image/"),
      );
      await handleFiles(files);
  }

  async function handleFiles(files) {
      uploadedFiles = [...uploadedFiles, ...files];
      document.getElementById("process").disabled =
          uploadedFiles.length < 2;

      const preview = document.getElementById("preview");

      for (const file of files) {
          const reader = new FileReader();
          reader.onload = async (e) => {
              const img = document.createElement("img");
              img.src = e.target.result;
              preview.appendChild(img);

              try {
                  await hdrProcessor.add_image(e.target.result);
              } catch (error) {
                  console.error("Error adding image:", error);
                  alert("Error adding image: " + error);
              }
          };
          reader.readAsDataURL(file);
      }
  }

  async function processHDR() {
      const processButton = document.getElementById("process");
      processButton.disabled = true;
      processButton.textContent = "Processing...";

      try {
          const result = await hdrProcessor.process_hdr();
          document.getElementById("result").src = result;
      } catch (error) {
          console.error("Error processing HDR:", error);
          alert("Error processing HDR: " + error);
      } finally {
          processButton.disabled = false;
          processButton.textContent = "Process HDR";
      }
  }

  initialize().catch(console.error);