import { inject } from '@angular/core';
import { CanActivateFn } from '@angular/router';
import { AuthService } from '../services/auth.service';

export const loginGuard: CanActivateFn = (_route, _state) => {
  return inject(AuthService).canActivateLogin();
};
