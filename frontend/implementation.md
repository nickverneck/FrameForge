# Frontend Implementation Guide

This guide provides instructions on how to interact with the FrameForge backend from a React frontend to generate images and check for available models.

## Checking for Available Models

To check for available models, you can make a GET request to the `/api/providers` endpoint. This will return a JSON object with the default provider and a list of available providers.

### Example in React:

```javascript
import React, { useState, useEffect } from 'react';

function ModelSelector() {
  const [providers, setProviders] = useState({ default: '', available: [] });

  useEffect(() => {
    fetch('http://localhost:8000/api/providers') // Replace with your backend URL
      .then(response => response.json())
      .then(data => setProviders(data))
      .catch(error => console.error('Error fetching providers:', error));
  }, []);

  return (
    <div>
      <h2>Available Models</h2>
      <p>Default: {providers.default}</p>
      <ul>
        {providers.available.map(provider => (
          <li key={provider}>{provider}</li>
        ))}
      </ul>
    </div>
  );
}

export default ModelSelector;
```

## Generating an Image

To generate an image, you need to send a POST request to the `/api/edit` endpoint. This request must be a `multipart/form-data` request and include the following fields:

- `images`: The image files to be edited.
- `prompt` (optional): A description of the desired style or modifications.
- `provider` (optional): The provider to use for image editing.

The response will be an image file.

### Example in React:

```javascript
import React, { useState } from 'react';

function ImageEditor() {
  const [selectedFiles, setSelectedFiles] = useState(null);
  const [prompt, setPrompt] = useState('');
  const [editedImage, setEditedImage] = useState(null);

  const handleFileChange = (event) => {
    setSelectedFiles(event.target.files);
  };

  const handlePromptChange = (event) => {
    setPrompt(event.target.value);
  };

  const handleSubmit = (event) => {
    event.preventDefault();

    if (!selectedFiles || selectedFiles.length === 0) {
      alert('Please select at least one image file.');
      return;
    }

    const formData = new FormData();
    for (let i = 0; i < selectedFiles.length; i++) {
      formData.append('images', selectedFiles[i]);
    }
    formData.append('prompt', prompt);

    fetch('http://localhost:8000/api/edit', { // Replace with your backend URL
      method: 'POST',
      body: formData,
    })
      .then(response => {
        if (!response.ok) {
          throw new Error('Image editing failed.');
        }
        return response.blob();
      })
      .then(blob => {
        setEditedImage(URL.createObjectURL(blob));
      })
      .catch(error => console.error('Error editing image:', error));
  };

  return (
    <div>
      <h2>Image Editor</h2>
      <form onSubmit={handleSubmit}>
        <div>
          <label>Images:</label>
          <input type="file" accept="image/*" multiple onChange={handleFileChange} />
        </div>
        <div>
          <label>Prompt:</label>
          <input type="text" value={prompt} onChange={handlePromptChange} />
        </div>
        <button type="submit">Generate Image</button>
      </form>
      {editedImage && (
        <div>
          <h3>Edited Image:</h3>
          <img src={editedImage} alt="Edited" />
        </div>
      )}
    </div>
  );
}

export default ImageEditor;
```
