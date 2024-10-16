
import React, { useState } from 'react';
import 'katex/dist/katex.min.css';
import { useSelector } from 'react-redux';
import * as api from '../../../api';

export default function User({ userData }) {
  let { username } = userData;
  return (<h1>{username}</h1>);
}