import { Routes } from '@angular/router';
import { LoginComponent } from '../login/login.component';
import { AccountComponent } from '../account/account.component';
import { loginGuard } from './guards/login.guard';
import { authGuard } from './guards/auth.guard';
import { RegisterTotpComponent } from './register-totp/register-totp.component';

export const routes: Routes = [
    { 
        path: 'login', 
        component: LoginComponent,
        canActivate: [loginGuard]
    },
    { 
        path: 'account',
        component: AccountComponent,
        canActivate: [authGuard]
    },
    { 
        path: 'register-totp',
        component: RegisterTotpComponent,
        canActivate: [authGuard]
    }
];
