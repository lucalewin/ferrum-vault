async function fetchServiceList() {
  try {
    // Make a GET request to the API
    const response = await fetch("/api/v1/password/list");

    // Check if the response is OK (status 200)
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    // Parse the JSON response
    const services = await response.json(); // Assuming the response is JSON

    // Get the <ul> element where the services will be displayed
    const serviceList = document.getElementById("service-list");

    // Clear any existing content in the list
    serviceList.innerHTML = "";

    // Loop through the services and create <li> elements for each
    services.forEach((service) => {
      const listItem = document.createElement("li"); // Create a <li> element
      listItem.textContent = service; // Set the text content to the service name
      serviceList.appendChild(listItem); // Append the <li> to the <ul>
    });
  } catch (error) {
    console.error("Failed to fetch service list:", error);
  }
}

document.getElementById("add_pwd").addEventListener("submit", async (e) => {
  e.preventDefault();

  const formData = new FormData(e.target);
  const data = JSON.stringify(Object.fromEntries(formData));
  console.log(data);
  const response = await fetch("/api/v1/password", {
    method: "post",
    headers: {
      "Content-Type": "application/json",
    },
    body: data,
  });
});

document.getElementById("get_pwd").addEventListener("submit", async (e) => {
  e.preventDefault();

  const formData = new FormData(e.target);
  const data = JSON.stringify(Object.fromEntries(formData));
  const response = await fetch("/api/v1/password/get", {
    method: "post",
    headers: {
      "Content-Type": "application/json",
    },
    body: data,
  });

  const json = await response.json();

  document.getElementById("output").innerHTML = JSON.stringify(json);
});

// Call the function when the page loads
window.onload = fetchServiceList;
