import { HttpClient } from '@angular/common/http';
import { Component } from '@angular/core';

@Component({
  selector: 'app-register-qrcode',
  imports: [],
  templateUrl: './register-totp.component.html',
  styleUrl: './register-totp.component.css'
})
export class RegisterTotpComponent {

  qrCode = '/api/totp/qrcode';
  totpMessage = '';

  constructor(private http: HttpClient) {
  }

  debugTotp() {
    this.http.get('/api/totp/debug-user-data').subscribe(data => {
      console.log(data);
    })
  }

  registerTotp() {
    this.http.post('/api/totp/set-secret', null).subscribe(() => {
      this.totpMessage = 'Your code has been successfully registered';
    })
  }


}
