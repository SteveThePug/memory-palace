import React, { useEffect } from 'react';
import User from './User/User.jsx';
import { useSelector, useDispatch } from 'react-redux';
import { usersGet } from '../../store/users.js';

export default function Users() {
  const dispatch = useDispatch();
  const users = useSelector((state) => state.users);

  useEffect(() => {
    dispatch(usersGet());
  }, [dispatch]);

  return (
    <div className='border-2 p-2'>
      {users.map((userData, index) => (
        <User userData={userData} key={index} />
      ))}
    </div>
  );
}