import React, { useEffect } from 'react';
import User from './User/User.jsx';
import { useSelector, useDispatch } from 'react-redux';
import { usersGet } from '../../store/users.js';

export default function Users() {
  const dispatch = useDispatch();
  const {users} = useSelector((state) => state);

  useEffect(() => {
    dispatch(usersGet());
  }, [dispatch]);

  return (
    <div>
    <h1>USERS</h1>
      {users.map((userData, index) => (
        <User userData={userData} key={index} />
      ))}
    </div>
  );
}