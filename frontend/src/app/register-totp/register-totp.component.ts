import { NgClass } from '@angular/common';
import { HttpClient, HttpErrorResponse } from '@angular/common/http';
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { catchError, throwError } from 'rxjs';

@Component({
  selector: 'app-register-qrcode',
  imports: [FormsModule, NgClass],
  templateUrl: './register-totp.component.html',
})
export class RegisterTotpComponent {

  qrCode = '/api/totp/qrcode';
  totpMessage = '';
  isErrorMessage = false;
  codeToCheck = '';

  constructor(private http: HttpClient) {
  }

  debugTotp() {
    this.http.get('/api/totp/debug-user-data').subscribe(data => {
      console.log(data);
    })
  }

  registerTotp() {
    this.http.post('/api/totp/set-secret', { code: this.codeToCheck })
      .pipe(catchError((error: HttpErrorResponse) => {
        if (error.status == 401) {
          this.totpMessage = 'Your code was not correct. Please try again.';
          this.isErrorMessage = true;
        } else {
          this.totpMessage = 'Something went very wrong';
          this.isErrorMessage = true;
        }

        return throwError(() => new Error('Cannot register code'))
      }))
      .subscribe(() => {
        this.totpMessage = 'Your code has been successfully registered';
    })
  }


}
