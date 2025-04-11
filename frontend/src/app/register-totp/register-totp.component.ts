import { Component } from '@angular/core';

@Component({
  selector: 'app-register-qrcode',
  imports: [],
  templateUrl: './register-totp.component.html',
  styleUrl: './register-totp.component.css'
})
export class RegisterTotpComponent {

  qrCode = "/api/qrcode";


}
