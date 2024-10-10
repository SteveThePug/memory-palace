import React from 'react';
import ReactDOM from 'react-dom/client'; // Updated import for React 18
import { BrowserRouter } from 'react-router-dom';

import './index.css'; // Importing your stylesheet

import App from './App';
import store from './store'

import { Provider } from 'react-redux'
import { GoogleOAuthProvider } from '@react-oauth/google';


const root = ReactDOM.createRoot(document.getElementById('root')); 

root.render(
  <React.StrictMode>
    <GoogleOAuthProvider clientId='701360193903-2d05m3t2r6fd0oq4am441dpj8am7l0tc.apps.googleusercontent.com'>
      <Provider store={store}>
        <BrowserRouter>
          <App />
        </BrowserRouter>
      </Provider>
    </GoogleOAuthProvider>
  </React.StrictMode>
);