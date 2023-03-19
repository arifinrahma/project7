<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>employeedetail</name>
   <tag></tag>
   <elementGuidId>8845b0ff-48d6-47b4-8050-c7edd6ff2dc3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${GlobalVariable.token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;id&quot;,
      &quot;value&quot;: &quot;1&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
      <webElementGuid>be36d4f7-f98d-43e5-884c-d006e6efb52b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.token}</value>
      <webElementGuid>0a38e9ce-4f0a-4775-8a75-669da03baa3c</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.baseurl}/api/v1/employee/1</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementPropertyValue(response, 'data.firstName', &quot;Boss&quot;)
WS.verifyElementPropertyValue(response, 'data.middleName', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'data.lastName', &quot;Besar&quot;)
WS.verifyElementPropertyValue(response, 'data.code', &quot;0001&quot;)
WS.verifyElementPropertyValue(response, 'data.employeeId', &quot;1&quot;)
WS.verifyElementPropertyValue(response, 'data.fullName', &quot;Boss Besar&quot;)
WS.verifyElementPropertyValue(response, 'data.status', null)
WS.verifyElementPropertyValue(response, 'data.dob', null)
WS.verifyElementPropertyValue(response, 'data.driversLicenseNumber', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'data.licenseExpiryDate', null)
WS.verifyElementPropertyValue(response, 'data.maritalStatus', null)
WS.verifyElementPropertyValue(response, 'data.gender', null)
WS.verifyElementPropertyValue(response, 'data.otherId', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'data.nationality', null)
WS.verifyElementPropertyValue(response, 'data.unit', null)
WS.verifyElementPropertyValue(response, 'data.jobTitle', null)
WS.verifyElementPropertyValue(response, 'data.supervisor', null)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
